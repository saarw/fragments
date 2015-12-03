import javassist.ClassPool;
import javassist.CtClass;
import javassist.CtMethod;
import javassist.NotFoundException;
import org.junit.Test;

/**
 * Created by williamsaar on 03/12/15.
 */
public class JavassistInvalidCastTest {
    @Test
    public void testCast() {
        ClassPool cp = ClassPool.getDefault();
        try {
            String code = "{ new JavassistInvalidCastTest().inspectReturn((Object) ($w) $_); } ";
            CtClass c = cp.get("JavassistInvalidCastTest$Target");
            for (CtMethod method : c.getDeclaredMethods()) {
                method.insertAfter(code);
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
    public void inspectReturn(String str) {}

    public void inspectReturn(Object obj) {}

    static class Target {
        public static byte[] arrayReturn() {
            return new byte[12];
        }

        public static int intReturn() {
            return 23;
        }
    }
}
