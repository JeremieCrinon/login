import { IsEmail, IsStrongPassword } from "class-validator";

export class ModifyNewAccountDto {
  @IsEmail()
  new_email: string;

  @IsStrongPassword()
  new_password: string;
}
