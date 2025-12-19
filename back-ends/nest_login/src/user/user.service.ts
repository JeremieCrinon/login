import { HttpException, HttpStatus, Injectable, NotFoundException } from '@nestjs/common';
import { CreateUserDto } from './dto/create-user.dto';
import { UpdateUserDto } from './dto/update-user.dto';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository, DataSource } from 'typeorm';
import { User, Role } from './entities/user.entity';
import * as bcrypt from 'bcrypt';
import { EmailService } from 'src/email/email.service';

@Injectable()
export class UserService {
  constructor(
    @InjectRepository(User)
    private usersRepository: Repository<User>,
    private dataSource: DataSource,
    private readonly emailService: EmailService
  ) { }

  async create(createUserDto: CreateUserDto): Promise<User> {
    return await this.dataSource.transaction(async manager => {

      // Verify the email isn't already taken
      if (await this.usersRepository.findOneBy({ email: createUserDto.email })) {
        throw new HttpException('This email is already taken', HttpStatus.CONFLICT);
      }

      // Create the new user
      const user = new User();
      user.email = createUserDto.email;

      // Get the roles from the Dto and add new_account if it's not already in
      let roles = createUserDto.roles;
      if (!roles.includes(Role.NEW_ACCOUNT)) roles.push(Role.NEW_ACCOUNT);
      user.role = roles;

      // Generate a random password and hash it, but keep the non-hashed version for sending it by email
      const password = Math.random().toString(36).slice(-12);
      const passwordHash = await bcrypt.hash(password, 10);
      user.password = passwordHash;

      // Save the user
      await manager.save(user);

      // Send the invite email
      await this.emailService.sendEmail({
        subject: "Create your new account",
        template: "emails/create-account",
        recipient: user.email,
        context: {
          email: user.email,
          password: password
        }
      })

      user.password = "secret"; // Change the password of the user so we don't return it, even hashed
      return user;
    })
  }

  async listRoles(): Promise<{ roles: Role[] }> {
    return { roles: Object.values(Role) };
  }

  async findAll(): Promise<{ users: User[] }> {
    const users: User[] = await this.usersRepository.find({ select: ['id', 'email', 'role'] });

    return { users: users };
  }

  async findOne(id: number): Promise<User> {
    const user = await this.usersRepository.findOneBy({ id: id });

    if (!user) {
      throw new NotFoundException();
    }

    return user;
  }

  update(id: number, updateUserDto: UpdateUserDto) {
    return `This action updates a #${id} user`;
  }

  remove(id: number) {
    return `This action removes a #${id} user`;
  }
}
