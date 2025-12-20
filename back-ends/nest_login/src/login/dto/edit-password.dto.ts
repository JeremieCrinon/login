import { IsString, IsStrongPassword } from "class-validator";

export class EditPasswordDto {
  @IsString()
  current_password: string;

  @IsStrongPassword()
  new_password: string;
}
