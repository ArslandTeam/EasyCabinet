import { useFormStatus } from "react-dom";

export function SubmitButton({ children, pendingText, style }: { children: React.ReactNode, pendingText: string, style?: string }) {
  const { pending } = useFormStatus();
  return (
    <button type="submit" disabled={pending} className={`${style}`} >
      {pending ? pendingText : children}
    </button>
  );
}
