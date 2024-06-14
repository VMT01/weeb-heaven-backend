import { INestApplication } from '@nestjs/common';
import { ConfigService } from '@nestjs/config';
import { NestFactory } from '@nestjs/core';
import { NestExpressApplication } from '@nestjs/platform-express';
import { DocumentBuilder, SwaggerModule } from '@nestjs/swagger';

import { EEnvKey, EValidNodeEnv } from '@constants/env.constant';

import { AppModule } from './app.module';

function initSwagger(app: INestApplication, path?: string) {
    const configBuilder = new DocumentBuilder()
        .setTitle('Weeb Heaven NodeJS Docs')
        .setDescription('Weeb Heaven API description for NodeJS')
        .setVersion('1.0')
        .addApiKey()
        .addBearerAuth()
        .addCookieAuth()
        .build();
    const document = SwaggerModule.createDocument(app, configBuilder);
    SwaggerModule.setup(path || 'api/docs', app, document);
}

async function bootstrap() {
    const app = await NestFactory.create<NestExpressApplication>(AppModule);
    app.enableCors();

    const configService = app.get(ConfigService);
    app.setGlobalPrefix(configService.get<string>(EEnvKey.API_PREFIX));
    if (configService.get<string>(EEnvKey.NODE_ENV) === EValidNodeEnv.DEVELOPMENT) {
        initSwagger(app, configService.get<string>(EEnvKey.SWAGGER_PATH));
    }

    await app.listen(configService.get<number>(EEnvKey.PORT));
    console.log(`Application is running on: ${await app.getUrl()}`);
}
bootstrap();
