import { hash, compare } from "bcrypt";

const SALT_ROUNDS = 10;

export const createHashedPassword = (password: string) => hash(password, SALT_ROUNDS);

export const comparePassword = (rawPassword: string, hashedPassword: string) => compare(rawPassword, hashedPassword);
