import { Request } from "express";
import { CanActivate, ExecutionContext, Injectable, UnauthorizedException } from "@nestjs/common";
import { Reflector } from "@nestjs/core";
import { JwtService } from "@nestjs/jwt";
import { IS_PUBLIC_KEY, TOKEN_COOKIE_KEY } from "#constants";
import { UserService } from "#user/user.service";
import { JWTUser } from "#types";

@Injectable()
export class AuthGuard implements CanActivate {
    constructor(
        private jwtService: JwtService,
        private reflector: Reflector,
        private userService: UserService,
    ) {}

    async canActivate(context: ExecutionContext): Promise<boolean> {
        const isPublic = this.reflector.getAllAndOverride<boolean>(IS_PUBLIC_KEY, [
            context.getHandler(),
            context.getClass(),
        ]);

        const request = context.switchToHttp().getRequest();
        const token = this.extractTokenFromHeader(request);

        if (!token && !isPublic) {
            throw new UnauthorizedException();
        }

        try {
            const payload: JWTUser = await this.jwtService.verifyAsync(token, {
                secret: process.env.JWT_SECRET,
            });
            const user = await this.userService.findOneBy({ _id: { $eq: payload.sub } });

            request["user"] = user;
        } catch {
            if (isPublic) {
                return true;
            }

            throw new UnauthorizedException();
        }

        return true;
    }

    private extractTokenFromHeader(request: Request): string | undefined {
        const token = request.cookies?.[TOKEN_COOKIE_KEY];
        return typeof token === "string" ? token : undefined;
    }
}
