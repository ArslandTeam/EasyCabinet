import { Notify } from "notiflix";

type NotifyType = "success" | "failure" | "warning" | "info";

export function failure(message: string) {
  showMessage("failure", message);
}

export function success(message: string) {
  showMessage("success", message);
}

function showMessage(type: NotifyType, message: string) {
  if (Array.isArray(message)) {
    return message.forEach((item) => _showMessage(type, item));
  }
  _showMessage(type, message);
}

function _showMessage(type: NotifyType, message: string) {
  Notify[type](message, {
    position: "right-bottom",
    fontSize: "14px",
  });
}
