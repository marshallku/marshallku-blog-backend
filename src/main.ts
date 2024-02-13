require("newrelic");

import { NestFactory } from "@nestjs/core";
import * as cookieParser from "cookie-parser";
import { AppModule } from "./app.module";

async function main() {
    const app = await NestFactory.create(AppModule, {
        cors: {
            credentials: true,
            origin: process.env.TRUSTED_DOMAINS?.split(","),
        },
    });

    if (process.env.NODE_ENV === "development") {
        const { SwaggerModule, DocumentBuilder } = await import("@nestjs/swagger");
        const config = new DocumentBuilder().setTitle("Blog api").setVersion("1.0").build();
        const document = SwaggerModule.createDocument(app, config);
        SwaggerModule.setup("docs", app, document);
    }

    app.use(cookieParser());

    await app.listen(process.env.NODE_PORT);
}

main();
