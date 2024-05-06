
import React from "react"

interface Props<T> {
  items: T[];
  children: (item: T) => React.ReactNode;
}

export const For = ({ items, children }: Props<any>) => {
  return (
    <>
      {items.map((item) => children(item))}
    </>
  )
}

export default For;
