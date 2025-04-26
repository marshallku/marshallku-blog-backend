import { BadRequestException, Injectable, UnauthorizedException } from "@nestjs/common";
import { JwtService } from "@nestjs/jwt";
import { UserService } from "#user/user.service";
import { comparePassword, createHashedPassword } from "#utils";

export interface TokenPayload {
    /** User ID */
    sub: string;
    /** User name */
    username: string;
}

@Injectable()
export class AuthService {
    constructor(
        private userService: UserService,
        private jwtService: JwtService,
    ) {}

    async generateToken(id: string, name: string) {
        const payload: TokenPayload = { sub: id, username: name };
        const token = await this.jwtService.signAsync(payload);

        return token;
    }

    async signIn(name: string, password: string) {
        const user = await this.userService.findOne(name);

        if (!user) {
            throw new UnauthorizedException();
        }

        const samePassword = await comparePassword(password, user.password);

        if (!samePassword) {
            throw new UnauthorizedException();
        }

        const token = await this.generateToken(user._id, user.name);

        return { user: user.toJSON(), token };
    }

    async signUp(name: string, password: string) {
        const existingUser = await this.userService.findOne(name);

        if (!name || !password) {
            throw new BadRequestException("Invalid input");
        }

        if (existingUser) {
            throw new BadRequestException("Already have same user with that name", {
                cause: new Error(),
                description: "Please sign up with another user name",
            });
        }

        const hashedPassword = await createHashedPassword(password);
        const user = await this.userService.create({ name, password: hashedPassword });
        const token = await this.generateToken(user._id, user.name);

        return { user: user.toJSON(), token };
    }
}
