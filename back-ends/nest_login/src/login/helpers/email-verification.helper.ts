import { Role, User } from "src/user/entities/user.entity";
import { EntityManager } from "typeorm";
import { Injectable } from "@nestjs/common";
import { EmailService } from 'src/email/email.service';
import { randomBytes } from "crypto";

@Injectable()
export class EmailVerificationHelper {
  constructor(
    private readonly emailService: EmailService
  ) { }

  async sendEmailVerification(manager: EntityManager, user: User,) {
    let roles = user.role;

    // Add the unverified email role to the user
    if (!roles.includes(Role.UNVERIFIED_EMAIL)) {
      roles.push(Role.UNVERIFIED_EMAIL);
    }

    const code = randomBytes(32).toString('base64url').slice(0, 32);

    user.role = roles;
    user.emailVerificationCode = code;

    await manager.save(user);

    await this.emailService.sendEmail({
      subject: "Verify your email adress",
      template: "emails/verify-email",
      recipient: user.email,
      context: {
        code: code
      }
    })
  }
}

