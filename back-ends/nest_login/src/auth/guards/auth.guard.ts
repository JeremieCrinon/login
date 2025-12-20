import {
  CanActivate,
  ExecutionContext,
  Injectable,
  UnauthorizedException
} from '@nestjs/common';
import { JwtService } from '@nestjs/jwt';
import { Request } from 'express';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { Role, User } from 'src/user/entities/user.entity';
import { Reflector } from '@nestjs/core';
import { REQUIRED_ROLE_KEY } from '../decorators/roles.decorator';

@Injectable()
export class AuthGuard implements CanActivate {
  constructor(
    @InjectRepository(User)
    private usersRepository: Repository<User>,
    private readonly jwtService: JwtService,
    private reflector: Reflector
  ) { }


  async canActivate(context: ExecutionContext): Promise<boolean> {
    const requiredRole = this.reflector.getAllAndOverride<Role>(REQUIRED_ROLE_KEY, [
      context.getHandler(),
      context.getClass(),
    ]);

    const request = context.switchToHttp().getRequest();
    const token = this.extractTokenFromHeader(request);

    if (!token) {
      throw new UnauthorizedException();
    }

    try {
      const payload = await this.jwtService.verifyAsync(token);

      const user = await this.usersRepository.findOneBy({ id: payload.sub });

      if (!user) {
        throw new UnauthorizedException();
      }

      request.user = user;

      if (!requiredRole) {
        return true; // If the required role isn't set, it means we just want to verify the user is logged in and get them in the request.user
      }

      // In the case the required role is either new_account or unverified_email
      if (requiredRole == Role.NEW_ACCOUNT || requiredRole == Role.UNVERIFIED_EMAIL) {
        if (user.role.includes(requiredRole)) { // If the user has the required role, we let them continue
          return true
        } else { // Else we just throw an unauthorized exception
          throw new UnauthorizedException();
        }
      }

      // Now, if the user has the new account or unverified email role, we throw an exception, as they only can go to routes for theses roles or routes with no roles required
      if (user.role.includes(Role.NEW_ACCOUNT) || user.role.includes(Role.UNVERIFIED_EMAIL)) {
        throw new UnauthorizedException();
      }

      // If the user has the admin role, they can go now, they have passed all the checks needed for them
      if (user.role.includes(Role.ADMIN)) {
        return true;
      }

      // If the required role is user, it means we just want users that have a verified account, we don't require them explicitly to have this role (it's useless as a user to have this role, it's just for the middleware)
      if (requiredRole == Role.USER) {
        return true;
      }

      // Now, we verify that the user has the require role
      if (!user.role.includes(requiredRole)) {
        throw new UnauthorizedException();
      }
    } catch {
      throw new UnauthorizedException();
    }

    // If we arrived here, it means the user is allowed to pass
    return true;
  }

  private extractTokenFromHeader(request: Request): string | undefined {
    const [type, token] = request.headers.authorization?.split(' ') ?? [];
    return type === 'Bearer' ? token : undefined;
  }
}
