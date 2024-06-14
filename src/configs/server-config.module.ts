import { Global, Module } from '@nestjs/common';
import { ConfigModule, ConfigService } from '@nestjs/config';
import Joi from 'joi';

import { EEnvKey, EValidNodeEnv } from '@constants/env.constant';

const validationSchema = Joi.object({
    [EEnvKey.NODE_ENV]: Joi.string()
        .valid(...Object.values(EValidNodeEnv))
        .default(EValidNodeEnv.DEVELOPMENT),
    [EEnvKey.PORT]: Joi.number().required(),
    [EEnvKey.API_PREFIX]: Joi.string(),
    [EEnvKey.SWAGGER_PATH]: Joi.string(),
});

@Global()
@Module({
    imports: [ConfigModule.forRoot({ validationSchema })],
    providers: [ConfigService],
    exports: [ConfigService],
})
export class ServerConfigModule {}
