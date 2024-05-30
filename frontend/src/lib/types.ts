
export type User = {
    id: string,
    username: string,
    email: string,
}

export enum ROLE {
    ADMINISTRATOR = "ADMINISTRATOR",
    USER = "USER"
}

export enum INSTANCETYPE {
    vm = "VIRTUAL-MACHINE",
    container = "CONTAINER"
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

export type CreateInstance = {
    name: string,
    owner: string,
    type: INSTANCETYPE,
    cpu: number,
    memory: number,
    storage: number,
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
