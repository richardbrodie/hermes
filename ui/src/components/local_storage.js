import JwtDecode from 'jwt-decode';

export function get_jwt() {
  return localStorage.getItem('token');
}
export function set_jwt(token) {
  localStorage.setItem('token', token);
}
export function remove_jwt() {
  localStorage.removeItem('token');
}
export function logged_in() {
  let token = get_jwt();
  if (token) { return true } else { return false }
}

export function access_token_str() {
  return `?access_token=${get_jwt()}`
}
export function token_data() {
  return JwtDecode(get_jwt());
}
