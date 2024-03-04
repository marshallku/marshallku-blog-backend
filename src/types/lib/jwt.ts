export interface JWTUser {
    sub: string;
    username: string;
    iat: number;
    exp: number;
}
