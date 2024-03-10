import { jest, test } from '@jest/globals';
import { optional, optionalAsync } from '../../bin/utils/option.mjs';

describe('option.mjs', () => {
    describe('optional()', () => {
        test('optional() accepts a fn, calls it, and returns the value in an option object.', () => {
            const mockFn = jest.fn(() => 22);
            const result = optional(mockFn);
            expect(mockFn).toHaveBeenCalledTimes(1);
            expect(result.data).toBe(22);
        });
        test('optional() gracefully handles an error and returns an option object', () => {
            const mockFn = jest.fn(() => { throw new Error('TestError'); });
            const result = optional(mockFn);
            expect(mockFn).toHaveBeenCalledTimes(1);
            expect(result.data).toBeNull();
            expect(result.error).not.toBeNull();
        });
    })
})

