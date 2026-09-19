<?php

/** The error PHP's date functions raise since 8.3; before that they returned false. */
class DateError extends Error
{
}

class DateException extends Exception
{
}

class DateMalformedStringException extends DateException
{
}

/**
 * The interface PHP's date classes share. Psalm's own stubs name it (DateTimeImmutable implements it)
 * without declaring it: in PHP it is reflected, and a compiled program has no reflection of it.
 */
interface DateTimeInterface
{
    // the format constants PHP's own DateTimeInterface carries: a compiled program has no constant
    // reflection, so they have to be declared to be seen
    public const ATOM = 'Y-m-d\\TH:i:sP';
    public const COOKIE = 'l, d-M-Y H:i:s T';
    public const ISO8601 = 'Y-m-d\\TH:i:sO';
    public const ISO8601_EXPANDED = 'X-m-d\\TH:i:sP';
    public const RFC822 = 'D, d M y H:i:s O';
    public const RFC850 = 'l, d-M-y H:i:s T';
    public const RFC1036 = 'D, d M y H:i:s O';
    public const RFC1123 = 'D, d M Y H:i:s O';
    public const RFC7231 = 'D, d M Y H:i:s \\G\\M\\T';
    public const RFC2822 = 'D, d M Y H:i:s O';
    public const RFC3339 = 'Y-m-d\\TH:i:sP';
    public const RFC3339_EXTENDED = 'Y-m-d\\TH:i:s.vP';
    public const RSS = 'D, d M Y H:i:s O';
    public const W3C = 'Y-m-d\\TH:i:sP';

    // without native return types, as Psalm's own stub of DateTimeImmutable declares them: a native
    // one here would be a signature mismatch against that stub
    /** @return string */
    public function format(string $format);

    /** @return int */
    public function getTimestamp();

    /** @return int */
    public function getOffset();

    /** @return DateInterval */
    public function diff(DateTimeInterface $targetObject, bool $absolute = false);
}

/** A timezone; Psalm's own stub describes it, and a compiled program needs the class to exist. */
class DateTimeZone
{
    public function __construct(private string $timezone = 'UTC')
    {
    }

    public function getName(): string
    {
        return $this->timezone;
    }
}

/** The difference between two dates; a compiled analyzer only needs its shape. */
class DateInterval
{
    public int $y = 0;
    public int $m = 0;
    public int $d = 0;
    public int $h = 0;
    public int $i = 0;
    public int $s = 0;
    public float $f = 0.0;
    public int $invert = 0;

    /** @var int|false */
    public int|false $days = false;

    public function __construct(string $duration = 'P0D')
    {
    }

    public function format(string $format): string
    {
        return '';
    }
}

/**
 * A minimal DateTime for the compiled analyzer: it only has to tell whether a modifier string is one PHP's
 * date parser accepts (DateTimeModifyReturnTypeProvider infers `DateTime|false` from that). Absolute dates are
 * not modelled; the recognised relative formats are the common ones ("+1 day", "next monday", "midnight", ...).
 *
 * A compiled program also reads this declaration as the description of the class, so every signature
 * here says what the call map says (`datetime::*`), not what the running PHP declares.
 */
class DateTime implements DateTimeInterface
{
    /** @return string|false */
    public function format(string $format)
    {
        return '';
    }

    /** @return int|false */
    public function getTimestamp()
    {
        return 0;
    }

    public function getOffset(): int
    {
        return 0;
    }

    public function diff(DateTimeInterface $targetObject, bool $absolute = false): DateInterval
    {
        return new DateInterval();
    }

    /** @return DateTimeZone|false */
    public function getTimezone()
    {
        return new DateTimeZone();
    }

    /** @return static */
    public function setTimezone(DateTimeZone $timezone): self
    {
        return $this;
    }

    /** @return static */
    public function add(DateInterval $interval): self
    {
        return $this;
    }

    /** @return static */
    public function sub(DateInterval $interval): self
    {
        return $this;
    }

    /** @return static */
    public function setTimestamp(int $timestamp): self
    {
        return $this;
    }

    /** @return static */
    public function setDate(int $year, int $month, int $day): self
    {
        return $this;
    }

    /** @return static */
    public function setTime(int $hour, int $minute, int $second = 0, int $microsecond = 0): self
    {
        return $this;
    }

    /** @return static */
    public static function createFromInterface(DateTimeInterface $object): self
    {
        return new static();
    }

    public function __construct(string $datetime = 'now', ?DateTimeZone $timezone = null)
    {
    }

    /**
     * Since PHP 8.3 an unparseable modifier raises rather than returning false, but the call map
     * still describes the return as `false|static`, so that is what this declares.
     *
     * @return static|false
     */
    public function modify(string $modifier)
    {
        $offset = $this->unparseableAt($modifier);
        if ($offset !== null) {
            $at = substr(trim($modifier), $offset, 1);
            throw new DateMalformedStringException(
                static::class . '::modify(): Failed to parse time string (' . $modifier . ') at position '
                    . $offset . ' (' . ($at === '' ? ' ' : $at) . ')',
            );
        }
        return $this;
    }

    /**
     * The offset where parsing gives up, or null when the whole string parses. PHP counts the offset
     * from the first non-blank character, as its own messages show.
     */
    private function unparseableAt(string $modifier): ?int
    {
        $m = strtolower(trim($modifier));
        if ($m === '') {
            return 0;
        }
        $unit = '(sec|second|min|minute|hour|day|week|fortnight|month|year|weekday|msec|millisecond|usec|microsecond)s?';
        $weekday = '(mon|tue|wed|thu|fri|sat|sun)[a-z]*';
        $keywords = ['now', 'today', 'tomorrow', 'yesterday', 'midnight', 'noon', 'first day of this month', 'last day of this month',
            'first day of next month', 'last day of next month', 'first day of last month', 'last day of last month'];
        if (in_array($m, $keywords, true)) {
            return null;
        }
        $parts = preg_split('/\s*,\s*|\s+(?=[+-]?\d)/', $m) ?: [];
        $seen = 0;
        foreach ($parts as $part) {
            if ($part === '') {
                continue;
            }
            $at = strpos($m, $part, $seen);
            $seen = $at === false ? $seen : $at + strlen($part);
            if (preg_match('/^[+-]?\d+\s*' . $unit . '(\s+ago)?$/', $part) === 1
                || preg_match('/^(next|last|this|previous)\s+(' . $unit . '|' . $weekday . ')$/', $part) === 1
                || preg_match('/^' . $weekday . '(\s+(next|last|this)\s+week)?$/', $part) === 1
                || preg_match('/^(first|last)\s+day\s+of(\s+(next|last|this|previous)\s+(month|year))?$/', $part) === 1
                || preg_match('/^(first|last|next|previous|second|third|fourth|fifth)\s+' . $weekday . '\s+of(\s+.*)?$/', $part) === 1
                || preg_match('/^\d{1,2}:\d{2}(:\d{2})?$/', $part) === 1
                || preg_match('/^\d{4}-\d{2}-\d{2}$/', $part) === 1
            ) {
                continue;
            }
            return $at === false ? 0 : $at;
        }
        return null;
    }
}
