import styled from 'styled-components';

const styled_a = `
a {
  color: black;
  text-decoration: none;
  outline: 0;
}
`;
const MainView = styled.div`
${ styled_a}
height: 99vh;
display: grid;
grid-template-rows: 6vh auto;
grid-template-columns: 230px 1fr;
`;

export { MainView }
