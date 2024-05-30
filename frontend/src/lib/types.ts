
export type Message = {
    id: string
    from: string
    message: string
    date: string
}

export type MessageData = {
    from: string
    message: string
}

export type User = {
    id: string,
    name: string,
    username: string,
    email: string,
    role: ROLE,
    n_instances: number,
    created_at: string
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

<<<<<<< HEAD
=======

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

>>>>>>> dev-auth
export type JwtPayload = {
    id: string,
    username: string,
    email: string,
    role: ROLE,
    exp: number
}

export type Instance = {
    id: string,
    name: string,
    ip_addresses: string[],
    specs: InstanceSpecs,
    cluster_node: string,
    user_id: string,
    type: string,
    status: string
}

export type InstanceSpecs = {
    cpu: number,
    ram: number,
    storage: number
}

export type PublicInstanceData = {
    name: string,
    specs: InstanceSpecs,
    cluster_node: string,
    user_id: string
}
