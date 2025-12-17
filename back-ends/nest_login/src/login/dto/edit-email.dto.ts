import { IsString, IsEmail } from "class-validator";

export class EditEmailDto {
  @IsEmail()
  new_email: string;

  @IsString()
  password: string;
}
