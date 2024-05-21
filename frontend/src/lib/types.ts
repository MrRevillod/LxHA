
export type User = {
    id: string,
    username: string,
    email: string,
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
