import * as bcrypt from "bcrypt";

const SALT_ROUNDS = 10;

export const createHashedPassword = (password: string) => bcrypt.hash(password, SALT_ROUNDS);

export const comparePassword = (rawPassword: string, hashedPassword: string) =>
    bcrypt.compare(rawPassword, hashedPassword);
