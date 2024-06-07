
import { z } from "zod"
import { ROLE, INSTANCETYPE } from "./types"

export const messageSchema = z.object({
    subject: z.string().min(1).max(100),
    body: z.string().min(1).max(500)
})

export const registerSchema = z.object({

    name: z.string()
        .min(5, { message: "Must be at least 5 characters" })
        .max(30, { message: "El nombre debe tener menos de 30 caracteres" }),

    username: z.string()
        .min(5, { message: "Must be at least 5 characters" })
        .max(20, { message: "El apodo debe tener menos de 30 caracteres" }),

    email: z.string()
        .email({ message: "El email no es válido" })
        .max(50, { message: "El email debe tener menos de 100 caracteres" }),

    role: z.nativeEnum(ROLE)
})

export const profileSchema = z.object({

    name:
        z.optional(z.string()).or(
            z.string()
                .min(5, { message: "El nombre debe tener al menos 5 caracteres" })
                .max(30, { message: "El nombre debe tener menos de 30 caracteres" })
        ),

    username:
        z.optional(z.string()).or(
            z.string()
                .min(5, { message: "El apodo debe tener al menos 5 caracteres" })
                .max(20, { message: "El apodo debe tener menos de 30 caracteres" })
        ),

    email:
        z.optional(z.string()).or(
            z.string()
                .email({ message: "El email no es válido" })
                .max(50, { message: "El email debe tener menos de 50 caracteres" })
        ),

    password:
        z.optional(z.string()).or(
            z.string()
                .min(8, { message: "La contraseña debe tener al menos 8 caracteres" })
                .max(30, { message: "La contraseña debe tener menos de 30 caracteres" })
                .regex(/^(?=.*\d)(?=.*[a-z])(?=.*[A-Z])(?=.*\W).{8,30}$/, { message: "La contraseña debe tener al menos un número, una letra mayúscula, una minúscula y un carácter especial" })
        ),

    confirmPassword:
        z.optional(z.string()).or(
            z.string()
                .min(8, { message: "La contraseña debe tener al menos 8 caracteres" })
                .max(30, { message: "La contraseña debe tener menos de 30 caracteres" })
                .regex(/^(?=.*\d)(?=.*[a-z])(?=.*[A-Z])(?=.*\W).{8,30}$/, { message: "La contraseña debe tener al menos un número, una letra mayúscula, una minúscula y un carácter especial" })
        )
})

    .refine((data) => !data.password || (data.password === data.confirmPassword), {
        message: "Las contraseñas no coinciden",
        path: ["confirmPassword"],
    })

export const emailSchema = z.object({
    email: z.string()
        .min(1, { message: "El correo electrónico es requerido" })
        .email({ message: "El correo electrónico no es válido" }),
})

export const passwordSchema = z.object({
    password: z.string()
        .min(8, { message: "La contraseña debe tener al menos 8 caracteres" })
        .max(30, { message: "La contraseña debe tener menos de 30 caracteres" })
        .regex(/^(?=.*\d)(?=.*[a-z])(?=.*[A-Z])(?=.*\W).{8,30}$/, { message: "La contraseña debe tener al menos un número, una letra mayúscula, una minúscula y un carácter especial" }),

    confirmPassword: z.string()
        .min(8, { message: "La contraseña debe tener al menos 8 caracteres" })
        .max(30, { message: "La contraseña debe tener menos de 30 caracteres" })
        .regex(/^(?=.*\d)(?=.*[a-z])(?=.*[A-Z])(?=.*\W).{8,30}$/, { message: "La contraseña debe tener al menos un número, una letra mayúscula, una minúscula y un carácter especial" })
})
    .refine((data) => data.password === data.confirmPassword, {
        message: "Las contraseñas no coinciden",
        path: ["confirmPassword"],
    })

export const loginFormSchema = z.object({

    email: z.string()
        .min(1, { message: "The email adress is required" })
        .email({ message: "Invalid email adress" }),

    password: z.string()
        .min(1, { message: "The password is required" })
})

export const instanceSchema = z.object({
    name: z.string(),
    owner: z.string(),
    cpu: z.number(),
    memory: z.number(),
    storage: z.number(),
    type: z.nativeEnum(INSTANCETYPE)
})
