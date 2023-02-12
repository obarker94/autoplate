import React from 'react';
import {Center} from './Center';
             
describe('Center', () => {
it('Center renders correctly', () => {
    cy.mount(<Center>Some Text</Center>);
    cy.get(`[data-cy=center]`).should(`exist`);
    cy.get(`[data-cy=center]`).should(`have.class`, `zzz`);
    });
})