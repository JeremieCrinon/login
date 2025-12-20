import { IsEmail } from "class-validator";

export class EditUserEmailDto {
  @IsEmail()
  email: string;
}
