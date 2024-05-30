
import { Link } from "react-router-dom"
import { Show } from "./Show"
import { EyeIcon } from "./Icons"
import { FieldError } from "react-hook-form"
import { useState, forwardRef } from "react"

type InputType = "text" | "password" | "email" | "number"

interface InputProps {
    label: string,
    type: InputType,
    placeholder?: string  ,
    error?: string | FieldError,
    name: string,
    islogin?: string | boolean,
    value?: string | number
}

export const Input = forwardRef<HTMLInputElement, InputProps>((props, ref) => {

    const { label, type, placeholder, error, name, islogin = false } = props

    const classes = `border-1 border-neutral-500 rounded-lg 
        p-2 focus:outline-none  focus:ring-blue-500 focus:border-blue-500 w-full pl-4 placeholder-neutral-400 text-neutral-950
    `

    const [inputType, setInputType] = useState<InputType>(type)

    const togglePasswordVisibility = () => {
        setInputType(inputType === "password" ? "text" : "password")
    }

    return (

        <div className="flex flex-col gap-3 w-full">

            <Show when={label === "Password" && type === "password" && islogin as boolean}>

                <div className="flex justify-between w-full items-center mt-2">

                    <label htmlFor={name} className="font-semibold text-neutral-950">{label}</label>

                    <Link to="/auth/reset-password" className="text-neutral-950 text-sm hover:underline hover:text-blue-500">
                        ¿Forgot your password?
                    </Link>

                </div>

            </Show>

            <Show when={label !== "Password" || type !== "password" || !islogin}>
                <label  >{label}</label>
            </Show>

            <div className="relative flex flex-row justify-center">

                <input ref={ref}
                    className={classes}
                    placeholder={placeholder}
                    {...props}
                    type={inputType}
                />

                <Show when={type === "password"} >
                    <EyeIcon open={inputType === "text"} onClick={togglePasswordVisibility} />
                </Show>

            </div>

            {error && <div className="text-red-500 text-sm">{error.toString()}</div>}

        </div>
    )
})

Input.displayName = "Input"
