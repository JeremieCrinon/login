import { Module } from "@nestjs/common";
import { JwtModule } from "@nestjs/jwt";
import { env } from "./env";

@Module({
  imports: [
    JwtModule.registerAsync({
      useFactory: () => ({
        global: true,
        secret: env("JWT_SECRET"),
        signOptions: { expiresIn: '7d' },
      })
    }),
  ],
  exports: [JwtModule],
})
export class SharedModule { }
