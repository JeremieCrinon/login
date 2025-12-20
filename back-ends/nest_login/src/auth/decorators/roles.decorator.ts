import { SetMetadata } from "@nestjs/common";
import { Role } from "src/user/entities/user.entity";

export const REQUIRED_ROLE_KEY = 'required_role';
export const RequiredRole = (required_role: Role) => SetMetadata(REQUIRED_ROLE_KEY, required_role);
