import { Module } from '@nestjs/common';

import CONFIGS from '@configs/index';

@Module({
    imports: [...CONFIGS],
})
export class AppModule {}
