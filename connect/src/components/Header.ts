import { html } from "lit";

export default function Header() {
  return html`<header class="dialog__header">
    <div class="dialog__logo">
      <svg viewBox="0 0 214 35" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path
          d="M37.35 5.89999C36.15 2.34999 32.8 -1.40722e-05 29.05 -1.40722e-05H19C15.25 -1.40722e-05 11.9 2.34999 10.7 5.89999L0.5 35H5.7L14.95 8.54998C15.7 6.39999 17.7 4.99999 19.95 4.99999H28.1C30.35 4.99999 32.35 6.39999 33.1 8.54998L42.35 35H47.55L37.35 5.89999Z"
          fill="var(--primary-color)"
        />
        <path
          d="M53.5508 28.75C53.5508 32.5 56.0508 35 59.8008 35H79.5508C89.9508 35 97.3008 27.85 97.3008 17.7V17.25C97.3008 7.09999 89.9508 -0.0500134 79.5508 -1.40722e-05H59.8008C56.0508 -1.40722e-05 53.5508 2.49999 53.5508 6.24999V28.75ZM61.3008 30.15C59.6508 30.15 58.5508 29.05 58.5508 27.4V7.54999C58.5508 5.89999 59.6508 4.79999 61.3008 4.79999H79.5008C87.0008 4.79999 92.3508 9.94999 92.3508 17.25V17.7C92.3508 25 87.0008 30.2 79.5008 30.15H61.3008Z"
          fill="var(--primary-color)"
        />
        <path
          d="M137.557 35H142.557V30H147.557V25H142.557V-1.40722e-05H137.557V25H111.607L124.507 -1.40722e-05H118.807L103.307 30H137.557V35Z"
          fill="var(--primary-color)"
        />
        <path
          d="M161.576 4.99999C163.226 4.99999 164.826 5.99999 165.526 7.49999L176.326 30.1C177.726 33.05 180.876 35 184.126 35C187.476 35 190.626 33.05 192.026 30.1L202.826 7.49999C203.526 5.99999 205.126 4.99999 206.776 4.99999H208.676V35H213.676V-1.40722e-05H207.176C203.426 -1.40722e-05 199.826 2.24999 198.226 5.64999L187.576 27.85C186.976 29.15 185.626 30 184.226 30C182.726 30 181.376 29.15 180.776 27.85L170.126 5.64999C168.526 2.24999 164.926 -1.40722e-05 161.176 -1.40722e-05H154.626V35H159.626V4.99999H161.576Z"
          fill="var(--primary-color)"
        />
      </svg>
    </div>
  </header>`;
}