import { base, Error } from "../api";

export type Auth = {
  authenticated: boolean;
  role?: string;
  email?: string;
};
export async function get_current_auth(): Promise<Auth | null> {
  let url = `${base}/admin/auth/get_current`;
  try {
    let res = await fetch(url, { credentials: "include" }).then((e) =>
      e.json(),
    );
    if (res.error) {
      return null;
    } else {
      return res as Auth;
    }
  } catch (e: any) {
    return null;
  }
}

export async function delete_current_auth(): Promise<boolean> {
  let url = `${base}/admin/auth/delete_current`;
  let error_title = "Erreur lors de la déconnexion";
  try {
    let res = await fetch(url, {
      credentials: "include",
      method: "DELETE",
    }).then((e) => e.json());
    if (res.error) {
      new Error(error_title, res.error);
      return false;
    } else {
      return true;
    }
  } catch (e: any) {
    new Error(error_title, e.toString());
    return false;
  }
}

export async function create_challenge(email: string): Promise<boolean> {
  let url = `${base}/admin/auth/challenge/create?email=${encodeURIComponent(email)}`;
  let error_title =
    "Erreur lors de la création du challenge d'authentification";
  try {
    let res = await fetch(url, {
      method: "POST",
      credentials: "include",
    }).then((e) => e.json());
    if (res.error) {
      new Error(error_title, res.error);
      return false;
    } else {
      return true;
    }
  } catch (e: any) {
    new Error(error_title, e.toString());
    return false;
  }
}

export async function verify_challenge(
  email: string,
  code: string,
): Promise<boolean> {
  let url = `${base}/admin/auth/challenge/verify?email=${encodeURIComponent(email)}&code=${encodeURIComponent(code)}`;
  let error_title =
    "Erreur lors de la vérification du challenge d'authentification";
  try {
    let res = await fetch(url, {
      credentials: "include",
    }).then((e) => e.json());
    if (res.error) {
      new Error(error_title, res.error);
      return false;
    } else {
      return true;
    }
  } catch (e: any) {
    new Error(error_title, e.toString());
    return false;
  }
}
