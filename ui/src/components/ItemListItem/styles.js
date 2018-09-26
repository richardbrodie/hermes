import styled from 'styled-components';
import { meta } from '../Elements'

const FeedItem = styled.div`
  font-weight: ${props => props.seen ? '400' : '700'};
  padding: 5px 0 5px 10px;
  border-bottom: 1px solid #e6e5e5;
  display: flex;
    &:hover {
    background: #EDEDED;
  }
`;

const Description = styled.span`
  ${ meta}
  flex: 1;
  color: #8b8b8b;
  font-size: 0.9em;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
`;

export { FeedItem, Description }
