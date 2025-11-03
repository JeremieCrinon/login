import { ISendMailOptions, MailerService } from '@nestjs-modules/mailer';
import { Injectable, Logger } from '@nestjs/common';

@Injectable()
export class EmailService {
  private readonly logger = new Logger(EmailService.name);

  constructor(private readonly mailerService: MailerService) { }

  async sendEmail(params: {
    subject: string;
    template: string;
    recipient: string;
    context: ISendMailOptions['context'];
  }) {
    try {
      const context = {
        ...params.context,
        logo_url: process.env.LOGO_URL,
        front_end_url: process.env.FRONT_END_URL,
        admin_email: process.env.ADMIN_EMAIL
      }
      const sendMailParams = {
        to: params.recipient,
        from: process.env.SMTP_FROM,
        subject: params.subject,
        template: params.template,
        context: context,
      };
      await this.mailerService.sendMail(sendMailParams);
    } catch (error) {
      this.logger.error(
        `Error while sending mail with the following parameters : ${JSON.stringify(
          params,
        )}`,
        error,
      );
      throw error
    }
  }
}
