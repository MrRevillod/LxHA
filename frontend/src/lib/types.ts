
export type User = {
    id: string,
    username: string,
    email: string,
    validated: boolean
}

export type LoginData = {
    username: string,
    password: string
}

export type RequestResetPasswordData = {
    email: string
}