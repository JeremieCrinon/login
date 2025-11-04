import { Injectable, UnauthorizedException } from '@nestjs/common';
import { LoginDto } from './dto/login.dto';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { User } from 'src/user/entities/user.entity';
import * as bcrypt from 'bcrypt';
import { JwtService } from '@nestjs/jwt';

@Injectable()
export class LoginService {
  constructor(
    @InjectRepository(User)
    private usersRepository: Repository<User>,
    private readonly jwtService: JwtService
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
}
