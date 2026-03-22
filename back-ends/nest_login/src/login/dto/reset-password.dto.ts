import { IsString } from "class-validator";

export class ResetPasswordDto {
  @IsString()
  code: string;

  @IsString()
  new_password: string;
}
