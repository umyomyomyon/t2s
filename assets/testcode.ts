describe('hoge function tests', () => {
    test('test1', () => {
      expect(func(1, 2)).toBe(3)
    })
  
    test('test2', () => {
      expect(func(82873, 987234)).toBe(1070107)
    })
  
    test('test3', () => {
      expect(func(0, 1)).toThrowError(ZeroError)
    })
  
    test('test4', () => {
      expect(func(1, 0)).toThrowError(ZeroError)
    })
  
    test('test5', () => {
      expect(func(0, 0)).toThrowError(ZeroError)
    })
  
    test('test6', () => {
      expect(func(-1, 1)).toBe(0)
    })
  
    test('test7', () => {
      expect(func(1, -1)).toBe(0)
    })
})


function hoge() {
  console.log('hoge')
}
describe('hoge function tests', () => {
  test('test1', () => {
    expect(func(1, 2)).toBe(3)
  })

  test('test2', () => {
    expect(func(82873, 987234)).toBe(1070107)
  })

  test('test3', () => {
    expect(func(0, 1)).toThrowError(ZeroError)
  })

  test('test4', () => {
    expect(func(1, 0)).toThrowError(ZeroError)
  })

  test('test5', () => {
    expect(func(0, 0)).toThrowError(ZeroError)
  })

  test('test6', () => {
    expect(func(-1, 1)).toBe(0)
  })

  test('test7', () => {
    expect(func(1, -1)).toBe(0)
  })
})

describe('hoge function tests', () => {
  test('test1', () => {
    expect(func(1, 2)).toBe(3)
  })

  test('test2', () => {
    expect(func(82873, 987234)).toBe(1070107)
  })

  test('test3', () => {
    expect(func(0, 1)).toThrowError(ZeroError)
  })

  test('test4', () => {
    expect(func(1, 0)).toThrowError(ZeroError)
  })

  test('test5', () => {
    expect(func(0, 0)).toThrowError(ZeroError)
  })

  test('test6', () => {
    expect(func(-1, 1)).toBe(0)
  })

  test('test7', () => {
    expect(func(1, -1)).toBe(0)
  })
})
  