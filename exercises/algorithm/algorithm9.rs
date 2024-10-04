/*
    stack
    This question requires you to use a stack to achieve a bracket match
*/

// 完善后的 Stack<T> 结构
#[derive(Debug)]
struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.last_mut()
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            stack: self.data.iter().collect(),
        }
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            stack: self.data.iter_mut().collect(),
        }
    }
}

struct IntoIter<T>(Stack<T>);

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

// 实现的 bracket_match 函数
fn bracket_match(bracket: &str) -> bool {
    let mut stack: Stack<char> = Stack::new();
    let bracket_pairs = vec![(')', '('), (']', '['), ('}', '{')];
    let closing_brackets: Vec<char> = bracket_pairs.iter().map(|(c, _)| *c).collect();
    let opening_brackets: Vec<char> = bracket_pairs.iter().map(|(_, c)| *c).collect();

    for ch in bracket.chars() {
        if opening_brackets.contains(&ch) {
            stack.push(ch);
        } else if closing_brackets.contains(&ch) {
            if let Some(top) = stack.pop() {
                // Find the matching opening bracket
                let expected = bracket_pairs
                    .iter()
                    .find(|&&(close, _)| close == ch)
                    .map(|&(_, open)| open);
                if expected != Some(top) {
                    return false;
                }
            } else {
                // Stack is empty but found a closing bracket
                return false;
            }
        }
        // Ignore non-bracket characters
    }

    // Stack should be empty if all brackets are matched
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bracket_matching_1() {
        let s = "(2+3){func}[abc]";
        assert_eq!(bracket_match(s), true);
    }
    #[test]
    fn bracket_matching_2() {
        let s = "(2+3)*(3-1";
        assert_eq!(bracket_match(s), false);
    }
    #[test]
    fn bracket_matching_3() {
        let s = "{{([])}}";
        assert_eq!(bracket_match(s), true);
    }
    #[test]
    fn bracket_matching_4() {
        let s = "{{(}[)]}";
        assert_eq!(bracket_match(s), false);
    }
    #[test]
    fn bracket_matching_5() {
        let s = "[[[]]]]]]]]]";
        assert_eq!(bracket_match(s), false);
    }
    #[test]
    fn bracket_matching_6() {
        let s = "";
        assert_eq!(bracket_match(s), true);
    }
}
