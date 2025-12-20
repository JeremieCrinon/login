import { IsString, IsStrongPassword } from "class-validator";

export class ResetPasswordDto {
  @IsString()
  code: string;

  @IsStrongPassword()
  new_password: string;
}
