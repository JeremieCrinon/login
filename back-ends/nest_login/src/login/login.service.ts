import { BadRequestException, ConflictException, Injectable, UnauthorizedException } from '@nestjs/common';
import { LoginDto } from './dto/login.dto';
import { ModifyNewAccountDto } from './dto/modify-new-account.dto';
import { VerifyEmailDto } from './dto/verify-email.dto';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository, Not, DataSource } from 'typeorm';
import { Role, User } from 'src/user/entities/user.entity';
import * as bcrypt from 'bcrypt';
import { JwtService } from '@nestjs/jwt';
import { EmailVerificationHelper } from './helpers/email-verification.helper';
import { ForgotPasswordDto } from './dto/forgot-password.dto';
import { randomBytes } from "crypto";
import { EmailService } from 'src/email/email.service';
import { env } from 'src/env';
import { ResetPasswordDto } from './dto/reset-password.dto';

@Injectable()
export class LoginService {
  constructor(
    @InjectRepository(User)
    private usersRepository: Repository<User>,
    private readonly jwtService: JwtService,
    private dataSource: DataSource,
    private emailVerificationHelper: EmailVerificationHelper,
    private readonly emailService: EmailService
  ) { }


  async login(loginDto: LoginDto): Promise<{ token: string }> {

    // Get the user with the email
    const user = await this.usersRepository.findOneBy({ email: loginDto.email })

    // Verify a user with the email exists
    if (!user) {
      throw new UnauthorizedException();
    }

    // Verify the password we got from the user is valid
    if (!await bcrypt.compare(loginDto.password, user.password)) {
      throw new UnauthorizedException(); // We return the same error as if the email is invalid for security reasons
    }

    // Generate the JWT payload
    const payload = { sub: user.id }; // Just put the id in the token, as we will get the full user in the auth middleware to always have it up to date (even if it gets modified after the token is generated)

    // Generate and return the JWT with the payload
    return { token: await this.jwtService.signAsync(payload) };

  }

  async userInfos(user: User) {
    return { roles: user.role, user_mail: user.email }
  }

  async modifyNewAccount(modifyNewAccountDto: ModifyNewAccountDto, user: User) {
    return await this.dataSource.transaction(async manager => {
      const emailConflictUser = await this.usersRepository.findOneBy({ email: modifyNewAccountDto.new_email, id: Not(user.id) })

      if (emailConflictUser) {
        throw new ConflictException("Email is already used by another account.");
      }

      const passwordHash = await bcrypt.hash(modifyNewAccountDto.new_password, 10);

      let roles = user.role;
      roles = roles.filter((role) => role !== Role.NEW_ACCOUNT);

      user.email = modifyNewAccountDto.new_email;
      user.password = passwordHash;
      user.role = roles;

      await manager.save(user);

      await this.emailVerificationHelper.sendEmailVerification(manager, user);

    })
  }

  async verifyEmail(verifyEmailDto: VerifyEmailDto, user: User) {
    if (verifyEmailDto.code !== user.emailVerificationCode) {
      throw new BadRequestException("The code you sent isn't the right one");
    }

    let roles = user.role;
    roles = roles.filter((role) => role !== Role.UNVERIFIED_EMAIL);

    user.role = roles;
    user.emailVerificationCode = null;

    await this.usersRepository.save(user);
  }

  async forgotPassword(forgotPasswordDto: ForgotPasswordDto) {
    return await this.dataSource.transaction(async manager => {
      // Get the user with the email
      const user = await this.usersRepository.findOneBy({ email: forgotPasswordDto.email })

      // Verify a user with the email exists
      if (!user) {
        return; // We do not return an error for security reasons
      }

      const code = randomBytes(32).toString('base64url').slice(0, 32);
      const link = `${env("FRONT_END_URL")}/forgot-password/${code}`;

      user.passwordResetCode = code;

      await manager.save(user);

      await this.emailService.sendEmail({
        subject: "Reset your password",
        template: "emails/forgot-password",
        recipient: user.email,
        context: {
          link: link
        }
      })
    })
  }

  async resetPassword(resetPasswordDto: ResetPasswordDto) {
    // Get the user by the code, if we get one it means the code is valid and we can continue with the user we got, else it means the code isn't valid
    const user = await this.usersRepository.findOneBy({ passwordResetCode: resetPasswordDto.code })

    if (!user) {
      throw new UnauthorizedException("The code you sent isn't valid");
    }

    const passwordHash = await bcrypt.hash(resetPasswordDto.new_password, 10);

    user.password = passwordHash;

    await this.usersRepository.save(user);
  }
}
