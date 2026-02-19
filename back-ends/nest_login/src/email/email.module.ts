import { Module } from '@nestjs/common';
import { EmailService } from './email.service';
import { EmailController } from './email.controller';
import { MailerModule } from '@nestjs-modules/mailer';
import { HandlebarsAdapter } from '@nestjs-modules/mailer/dist/adapters/handlebars.adapter';
import { env } from 'src/env';

@Module({
  imports: [
    MailerModule.forRootAsync({
      useFactory: () => ({
        transport: {
          host: env("SMTP_HOST"),
          port: +env("SMTP_PORT"),
          secure: false,
          tls: {
            rejectUnauthorized: false,
          },
          auth: {
            user: env("SMTP_USER"),
            pass: env("SMTP_PASS")
          }
        },
        defaults: {
          from: env("SMTP_FROM"),
        },
        template: {
          dir: __dirname + '/../../templates',
          adapter: new HandlebarsAdapter(),
          options: {
            strict: true,
          },
        },
      }),
    }),
  ],
  providers: [EmailService],
  exports: [EmailService],
  controllers: [EmailController]
})
export class EmailModule { }
