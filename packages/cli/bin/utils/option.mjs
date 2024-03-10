export function optional(callback) {
  let data;
  let error = null;
  try {
    data = callback();
  } catch (e) {
    data = null;
    error = e;
  } finally {
    return {
      data,
      error
    };
  }
}

export async function optionalAsync(callback) {
  let data;
  let error = null;
  try {
    data = await callback();
  } catch (e) {
    data = null;
    error = e;
  } finally {
    return {
      data,
      error
    };
  }
}
