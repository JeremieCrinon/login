import { IsEmail, IsEnum } from "class-validator";
import { Role } from '../entities/user.entity';

export class CreateUserDto {
  @IsEmail()
  email: string;

  @IsEnum(Role, { each: true })
  roles: Role[];
}
