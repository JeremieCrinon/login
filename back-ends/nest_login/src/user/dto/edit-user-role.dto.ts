import { IsEnum } from "class-validator";
import { Role } from '../entities/user.entity';

export class EditUserRoleDto {
  @IsEnum(Role, { each: true })
  roles: Role[];
}
