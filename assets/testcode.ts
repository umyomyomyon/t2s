import { ZeroError } from './error';

// 引数の一方が0の場合はエラーを投げる, それ以外は足し算を行う
describe('hoge function tests', () => {
    test('test1', () => {
      expect(hoge(1, 2)).toBe(3)
    })
  
    test('test2', () => {
      expect(hoge(82873, 987234)).toBe(1070107)
    })
  
    test('test3', () => {
      expect(hoge(0, 1)).toThrowError(ZeroError)
    })
  
    test('test4', () => {
      expect(hoge(1, 0)).toThrowError(ZeroError)
    })
  
    test('test5', () => {
      expect(hoge(0, 0)).toThrowError(ZeroError)
    })
  
    test('test6', () => {
      expect(hoge(-1, 1)).toBe(0)
    })
  
    test('test7', () => {
      expect(hoge(1, -1)).toBe(0)
    })
})


function hoge() {
  console.log('hoge')
}
describe('hoge function tests', () => {
  test('test1', () => {
    expect(fuga(1, 2)).toBe(3)
  })
})
  