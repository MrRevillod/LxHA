
import React, { useState } from "react";

import { EyeIcon } from "./Icons.js";
import { Link } from "react-router-dom";
import { FieldError } from "react-hook-form";

interface InputProps {
    label: string;
    type: "text" | "password" | "email"; // Define allowed input types
    placeholder?: string;
    error?: string | FieldError;
    name: string;
    islogin?: string; // Use optional chaining for islogin
}

export const Input = React.forwardRef<HTMLInputElement, InputProps>((props, ref) => {
    const { label, type, placeholder, error, name, islogin = false } = props;

    const classes = `bg-neutral-950 border-1 border-neutral-500 rounded-lg p-2
                 focus:outline-none focus:ring-2 focus:ring-neutral-500
                 h-12 w-full pl-4 placeholder-neutral-400 text-neutral-100
  `;

    const [inputType, setInputType] = useState<"text" | "password" | "email">(type);

    const togglePasswordVisibility = () => {
        setInputType(inputType === "password" ? "text" : "password");
    };

    return (
        <div className="flex flex-col gap-3 w-full">
            {(label === "Contraseña" && type === "password" && islogin) && (
                <div className="flex justify-between w-full items-center mt-2">
                    <label htmlFor={name} className="font-semibold text-neutral-100">
                        {label}
                    </label>
                    <Link to="/auth/reset-password" className="text-neutral-100 text-sm hover:underline hover:text-blue-500">
                        ¿Olvidaste tu contraseña?
                    </Link>
                </div>
            )}

            <label htmlFor={name} className="font-semibold text-neutral-100">
                {label}
            </label>

            <div className="relative flex flex-row justify-center">
                <input
                    ref={ref}
                    className={classes}
                    placeholder={placeholder}
                    {...props} // Spread all other props
                    type={inputType}
                />

                {type === "password" && (
                    <EyeIcon
                        open={inputType === "text"}
                        onClick={togglePasswordVisibility}
                    />
                )}
            </div>

            {error && <div className="text-red-500 text-sm">{error.toString()}</div>}
        </div>
    );
});

Input.displayName = "Input";
