import { Module } from "@nestjs/common";
import * as Joi from "joi";
import { ConfigModule } from "@nestjs/config";
import { MongooseModule } from "@nestjs/mongoose";
import { AppController } from "./app.controller";
import { AppService } from "./app.service";
import { AuthController } from "./auth/auth.controller";
import { AuthModule } from "./auth/auth.module";
import { UserController } from "./user/user.controller";
import { UserModule } from "./user/user.module";

@Module({
    imports: [
        ConfigModule.forRoot({
            isGlobal: true,
            envFilePath: `.env`,
            validationSchema: Joi.object({
                NODE_PORT: Joi.string().required(),
                MONGO_PORT: Joi.string().required(),
                MONGO_HOST: Joi.string().required(),
            }),
        }),
        MongooseModule.forRoot(`mongodb://${process.env.MONGO_HOST}:${process.env.MONGO_PORT}/`, {
            connectionName: "default",
        }),
        AuthModule,
        UserModule,
    ],
    controllers: [AppController, AuthController, UserController],
    providers: [AppService],
})
export class AppModule {}
