import { IsEmail, IsString } from "class-validator";

export class ModifyNewAccountDto {
  @IsEmail()
  new_email: string;

  @IsString()
  new_password: string;
}
