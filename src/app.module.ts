import { Module } from "@nestjs/common";
import * as Joi from "joi";
import { ConfigModule } from "@nestjs/config";
import { MongooseModule } from "@nestjs/mongoose";
import { AppController } from "./app.controller";
import { AppService } from "./app.service";
import { AuthController } from "./auth/auth.controller";
import { AuthModule } from "./auth/auth.module";

@Module({
    imports: [
        ConfigModule.forRoot({
            isGlobal: true,
            envFilePath: `.env`,
            validationSchema: Joi.object({
                NODE_PORT: Joi.string().required(),
                MONGO_PORT: Joi.string().required(),
                MONGO_HOST: Joi.string().required(),
                MONGO_USERNAME: Joi.string().required(),
                MONGO_PASSWORD: Joi.string().required(),
            }),
        }),
        MongooseModule.forRoot(
            `mongodb://${process.env.MONGO_USERNAME}:${process.env.MONGO_PASSWORD}@${process.env.MONGO_HOST}:${process.env.MONGO_PORT}/`,
            {
                connectionName: "default",
            },
        ),
        AuthModule,
    ],
    controllers: [AppController, AuthController],
    providers: [AppService],
})
export class AppModule {}
