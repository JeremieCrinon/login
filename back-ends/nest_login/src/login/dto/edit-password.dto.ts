import { IsString } from "class-validator";

export class EditPasswordDto {
  @IsString()
  current_password: string;

  @IsString() // We do not use isStrongPassword from class-validator as it can cause conflicts with front-end validation and it isn't necessary (if a user wants to manipulate the frontend to use a weak password, so be it)
  new_password: string;
}
