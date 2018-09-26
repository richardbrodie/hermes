import styled from 'styled-components';
import TimeAgo from 'react-timeago';

const meta = `
  color: #8b8b8b;
  font-size: 0.8em;
  border-left: 1px solid #0a0a0a;
  margin-left: 10px;
  padding-left: 10px;
  padding-right: 5px;
`;

const StyledTime = styled(TimeAgo)`
  ${ meta}
  min-width: 85px;
  margin-left: auto;
`;

const Form = styled.div`
  font-family: "Roboto", sans-serif;
  font-size: 14px;
  width: 360px;
  margin: 0 auto 100px;
  position: relative;
  z-index: 1;
  background: #ffffff;
  max-width: 360px;
  padding: 45px;
  text-align: center;
`;

const Input = styled.input`
  outline: 0;
  background: #f2f2f2;
  width: 100%;
  border: 0;
  margin: 0 0 15px;
  padding: 15px;
  box-sizing: border-box;
`;

const Button = styled.button`
  border: 1px solid #3498db;
  border-radius: 3px;
  background-color: #3498db;
  color: white;
  display: inline-block;
  text-transform:uppercase;
  font-weight: 700;
  line-height: 22px;
  padding: 0.5em 1em;
  min-width: 5em;
  &:hover {
      background-color: #2a7aaf;
  }
`;

export { meta, Form, Input, Button, StyledTime }
