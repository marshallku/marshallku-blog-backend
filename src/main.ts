import { NestFactory } from "@nestjs/core";
import { SwaggerModule, DocumentBuilder } from "@nestjs/swagger";
import { AppModule } from "./app.module";

async function main() {
    const app = await NestFactory.create(AppModule);

    const config = new DocumentBuilder().setTitle("Blog api").setVersion("1.0").build();
    const document = SwaggerModule.createDocument(app, config);
    SwaggerModule.setup("docs", app, document);

    await app.listen(process.env.NODE_PORT);
}

main();
