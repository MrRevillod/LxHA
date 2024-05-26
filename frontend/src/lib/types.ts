
export type User = {
    id: string,
    name: string,
    username: string,
    email: string,
    role: ROLE,
    instances: string[],
    nInstances: number
}

export enum ROLE {
    ADMINISTRATOR = "ADMINISTRATOR",
    USER = "USER"
}

export type RegisterData = {
    username: string,
    email: string,
    password: string,
    confirmPassword: string,
    role: ROLE
}

export type LoginData = {
    username: string,
    password: string
}

export type RequestResetPasswordData = {
    email: string
}

export type Instance = {
    id: string,
    name: string
}

export type PublicInstanceData = {
    name: string
}

export type JwtPayload = {
    id: string,
    username: string,
    email: string,
    role: ROLE,
    exp: number
}
