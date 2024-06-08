
import { FieldError } from "react-hook-form"
import { forwardRef, LegacyRef } from "react"

interface InputProps {
    label: string,
    placeholder?: string,
    error?: string | FieldError,
}

import "../../index.css"

export const TextArea = forwardRef<HTMLInputElement, InputProps>((props, ref) => {

    const { label, placeholder, error } = props

    const classes = `border-1 ${error ? "border-red-400" : "border-neutral-500"} rounded-lg 
        p-2 focus:outline-none  focus:ring-blue-500 focus:border-blue-500 w-full 
        pl-4 placeholder-neutral-400 text-neutral-950
    `

    return (

        <div className="flex flex-col gap-3 w-full">

            <div className="flex flex-row gap-2 items-center justify-between">
                <label className="font-medium">{label}</label>
                {error && <div className="text-red-600 text-sm">{error.toString()}</div>}
            </div>

            <div className="relative flex flex-row justify-center">

                <textarea ref={ref as LegacyRef<HTMLTextAreaElement>}
                    className={`${classes} text-area h-32 resize-none`}
                    placeholder={placeholder}
                    {...props}
                />

            </div>

        </div>
    )
})

TextArea.displayName = "TextArea"
