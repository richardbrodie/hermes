import styled from 'styled-components';
import { Link } from "@reach/router";

const Logo = styled.img`
max-height: 35px;
padding-right: 0.2em;
vertical-align: text-top;
`;

const StyledSidebar = styled.div`
border-right: 1px solid #0a0a0a;
background-color:#EDEDED;
grid-row: 1 / span 2;
grid-column: 1;
display: flex;
flex-direction: column;
`;

const StyledTopBar = styled.div`
padding: 0.5em;
text-transform: uppercase;
text-align: center;
cursor: pointer;
font-size: 1.3em;
border-bottom: 1px solid #d1cfcf;
`;

const StyledFeedList = styled.nav`
grid-row: 2;
grid-column: 1;
font-size: 10pt;
`;

const AddFeedButton = styled.div`
margin-left: auto;
margin-right: auto;
margin-top: auto;
`;

const FeedLink = styled(Link)`
padding: 10px 10px;
border-bottom: 1px solid #d1cfcf;
display: flex;
&:hover {
background: rgb(255, 255, 255);
}
&.active {
font-weight: 600;
border-left: 0.5em solid #2cadee;
}
`;

const FeedCount = styled.span`
margin-left: auto;
margin-top: auto;
margin-bottom: auto;
font-weight: 400;
border: 1px solid;
border-radius: 7px;
background: #2cadee;
color: white;
padding: 0 8px;
`;

const MenuLink = styled(Link)`
text-transform: uppercase;
cursor: pointer;
text-align: center;
padding-left: 10px;
padding-right: 10px;
&:hover {
background: rgb(255, 255, 255);
}
`;

export {
  Logo,
  StyledSidebar,
  StyledTopBar,
  StyledFeedList,
  AddFeedButton,
  FeedLink,
  FeedCount,
  MenuLink
}
